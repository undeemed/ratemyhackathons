# Lessons Learned

<!-- Format: date, what went wrong, rule for next time -->
<!-- Claude reads this at session start and applies before touching code -->

| Date | What Went Wrong | Rule |
|------|----------------|------|
| 2026-03-15 | Canvas `h-full` (height:100%) doesn't resolve against parent's `aspect-ratio`-derived height → oval globe | Use `aspect-square` on the canvas itself for self-sizing, never rely on `h-full` with `aspect-ratio` parents |
| 2026-03-15 | Cobe used `offsetWidth` for width and `offsetHeight` for height — any CSS mismatch caused oval | Always use `offsetWidth` for BOTH cobe width and height (official cobe pattern) |
| 2026-03-15 | Globe.svelte `onMount` fires before parent's `onMount` (Svelte child-before-parent) → cobe created with 0x0 canvas | Wrap cobe `createGlobe` in `requestAnimationFrame` to ensure parent GSAP dimensions are set first |
| 2026-03-15 | Removed `aspect-square` from globe wrapper → wrapper collapsed to 0x0 → globe disappeared | Removing CSS sizing without providing alternative dimensions causes element collapse; always verify elements have a size source |
| 2026-03-15 | Pixel-based `left` centering drifted with GSAP `scale` transforms | Use `left: '50%'` + `xPercent: -50` for centering — works correctly with all transforms |
| 2026-03-15 | Per-event zoom in/out (scale 1.25→1) caused distracting pulsing during scrub | Apply zoom once at section start, hold during content cycling, zoom out once at exit |
| 2026-03-15 | Pushed to prod before user tested locally — broke globe sizing | ALWAYS let user test on localhost first before pushing/deploying |
| 2026-03-15 | Capped cobe's cached `width` variable to 800px — broke display size (cobe rendered 800px but CSS stretched to 1440px) | Cap `devicePixelRatio` instead of width to limit GPU buffer; width must match actual display size for correct rendering |
| 2026-03-15 | Transform-based morph (`scale`/`x`/`y`) caused globe to overflow container — cobe renders at original size regardless of CSS transform | Use layout-based GSAP morph (`width`/`height`/`left`) so cobe re-renders at correct size within its container |
