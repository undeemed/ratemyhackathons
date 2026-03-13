"""OpenRouter LLM client — dynamic free model discovery + paid fallback.

Strategy:
1. Discover all free models from /api/v1/models (cached 10min)
2. Sort by provider reliability (Google > Mistral > OpenAI > others)
3. Try top 4 reliable + 4 random free models
4. If all free models fail, fall back to cheap paid model
"""

import json
import os
import random
import time
import urllib.request
import urllib.error

from dotenv import load_dotenv

load_dotenv()

API_URL = "https://openrouter.ai/api/v1/chat/completions"
MODELS_URL = "https://openrouter.ai/api/v1/models"

# Cheap paid model as final fallback (~$0.10/1M tokens)
PAID_FALLBACK = "google/gemini-2.0-flash-001"

# Cache of discovered free models
_free_models: list[str] = []
_models_fetched_at: float = 0

# Provider reliability ranking for free tier
_PROVIDER_PRIORITY = {
    "google": 0,
    "mistralai": 1,
    "openai": 2,
    "qwen": 3,
    "stepfun": 4,
}


def _get_api_key() -> str | None:
    return os.getenv("OPENROUTER_API_KEY")


def _provider_sort_key(model_id: str) -> tuple[int, str]:
    provider = model_id.split("/")[0] if "/" in model_id else ""
    return (_PROVIDER_PRIORITY.get(provider, 10), model_id)


def _fetch_free_models() -> list[str]:
    """Discover free models from OpenRouter API."""
    global _free_models, _models_fetched_at

    if _free_models and (time.time() - _models_fetched_at) < 600:
        return _free_models

    try:
        req = urllib.request.Request(MODELS_URL, method="GET")
        with urllib.request.urlopen(req, timeout=15) as resp:
            data = json.loads(resp.read().decode())

        models = []
        for m in data.get("data", []):
            pricing = m.get("pricing", {})
            if str(pricing.get("prompt", "1")) != "0":
                continue

            model_id = m.get("id", "")
            if model_id.startswith("openrouter/"):
                continue

            arch = m.get("architecture", {})
            if "text" not in arch.get("output_modalities", []):
                continue

            models.append(model_id)

        models.sort(key=_provider_sort_key)
        _free_models = models
        _models_fetched_at = time.time()
        return _free_models

    except Exception:
        return [
            "google/gemma-3-27b-it:free",
            "google/gemma-3-12b-it:free",
            "google/gemma-3n-e2b-it:free",
        ]


def _call_model(
    model: str,
    messages: list[dict],
    api_key: str,
    max_tokens: int,
) -> str | None:
    """Make a single API call to a model. Returns content or None."""
    payload = {
        "model": model,
        "messages": messages,
        "max_tokens": max_tokens,
        "temperature": 0.1,
    }

    headers = {
        "Authorization": f"Bearer {api_key}",
        "Content-Type": "application/json",
        "HTTP-Referer": "https://ratemyhackathons.com",
        "X-Title": "RateMyHackathons Crawler",
    }

    req = urllib.request.Request(
        API_URL,
        data=json.dumps(payload).encode(),
        headers=headers,
        method="POST",
    )

    try:
        with urllib.request.urlopen(req, timeout=45) as resp:
            raw = resp.read().decode()
            if not raw:
                return None
            data = json.loads(raw)
            choices = data.get("choices")
            if not choices:
                return None
            msg = choices[0].get("message")
            if not msg:
                return None
            content = msg.get("content")
            return content.strip() if content else None
    except urllib.error.HTTPError as e:
        try:
            _ = e.read()
        except Exception:
            pass
        return None
    except Exception:
        return None


def ask(
    prompt: str,
    system: str = "",
    max_tokens: int = 1024,
) -> str | None:
    """Send a prompt to OpenRouter. Tries free models first, then paid.

    Free: rotates through discovered models (Google-priority).
    Paid: falls back to gemini-2.0-flash if all free models fail.
    """
    api_key = _get_api_key()
    if not api_key:
        return None

    messages = []
    if system:
        messages.append({"role": "system", "content": system})
    messages.append({"role": "user", "content": prompt})

    # --- Try free models first ---
    models = _fetch_free_models()
    if models:
        top = models[:4]
        rest = models[4:]
        if rest:
            random.shuffle(rest)
            candidates = top + rest[:4]
        else:
            candidates = top

        for model in candidates:
            result = _call_model(model, messages, api_key, max_tokens)
            if result:
                return result

    # --- Paid fallback ---
    result = _call_model(PAID_FALLBACK, messages, api_key, max_tokens)
    if result:
        return result

    return None
