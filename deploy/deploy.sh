#!/bin/bash
set -euo pipefail

# Deploy RateMyHackathons to Mac Mini via Tailscale
# Usage: ./deploy.sh [--build-only]

REMOTE_USER="${DEPLOY_USER:-lexie}"
REMOTE_IP="${DEPLOY_IP:-100.77.36.51}"
SSH_KEY="${DEPLOY_SSH_KEY:-$HOME/.ssh/macmini}"
REMOTE_DIR="~/ratemyhackathons"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

SSH_CMD="ssh -i $SSH_KEY $REMOTE_USER@$REMOTE_IP"
SCP_CMD="scp -i $SSH_KEY"
RSYNC_SSH="ssh -i $SSH_KEY"

echo "=== RateMyHackathons Deploy ==="
echo "Target: $REMOTE_USER@$REMOTE_IP"
echo "Project: $PROJECT_DIR"

# Check .env exists
if [ ! -f "$SCRIPT_DIR/.env" ]; then
    echo "ERROR: deploy/.env not found. Copy .env.example and fill in your values:"
    echo "  cp deploy/.env.example deploy/.env"
    exit 1
fi

# Sync project to Mac Mini (excluding build artifacts)
echo ""
echo "--- Syncing project to $REMOTE_IP ---"
rsync -avz --delete \
    -e "$RSYNC_SSH" \
    --exclude 'node_modules' \
    --exclude '.venv' \
    --exclude 'target' \
    --exclude '.git' \
    --exclude '*.pyc' \
    --exclude '__pycache__' \
    --exclude '.DS_Store' \
    --exclude 'deploy/.env' \
    --exclude 'frontend/build' \
    --exclude '.svelte-kit' \
    "$PROJECT_DIR/" "$REMOTE_USER@$REMOTE_IP:$REMOTE_DIR/"

# Copy .env separately (not deleted by --delete)
echo ""
echo "--- Copying .env ---"
$SCP_CMD "$SCRIPT_DIR/.env" "$REMOTE_USER@$REMOTE_IP:$REMOTE_DIR/deploy/.env"

if [ "${1:-}" = "--build-only" ]; then
    echo ""
    echo "--- Building containers (no restart) ---"
    $SSH_CMD "cd $REMOTE_DIR/deploy && docker compose build"
else
    echo ""
    echo "--- Building and starting containers ---"
    $SSH_CMD "cd $REMOTE_DIR/deploy && docker compose up -d --build"

    echo ""
    echo "--- Waiting for services ---"
    sleep 5

    echo ""
    echo "--- Container status ---"
    $SSH_CMD "cd $REMOTE_DIR/deploy && docker compose ps"
fi

echo ""
echo "=== Deploy complete ==="
echo "Frontend: http://$REMOTE_IP:3000"
echo "Backend:  http://$REMOTE_IP:8080/health"
echo "DB:       psql -h $REMOTE_IP -U rmh -d ratemyhackathons"
