# Demo Assets

This directory contains animated GIF demonstrations and screenshots.

**[View All Demos in Gallery →](../DEMOS.md)**

## Files

### Screenshots
- `dtree_screenshot.png` — Main screenshot

### Animated Demos
- `tree_navigation.gif` — Tree navigation demo
- `search.gif` — Search functionality demo
- `bookmarks.gif` — Bookmarks demo

### Old/Unused Files
- `file_viewer.gif` — Leftover from dtree (file viewer removed)
- `visual_selection.gif` — Leftover from dtree (visual selection removed)

## Generation

Generate all demos:
```bash
cd ../../demos
./generate_all.sh
```

See `demos/README.md` for detailed instructions.

## File Size

Keep GIF files under 5MB for fast loading:
- Use optimized settings in VHS tapes
- Limit demo duration to 10–15 seconds
- Optimize with gifsicle if needed:
  ```bash
  gifsicle -O3 --lossy=80 input.gif -o output.gif
  ```

## Notes

- GIFs are tracked in git (included in repo)
- Regenerate when features change
- Test GIFs display correctly on GitHub before committing
