# Dragon Island Blue - Custom touchHLE Build

This repository contains a specially optimized and patched build of the [touchHLE emulator](https://touchhle.org/), specifically tailored for playing **Dragon Island Blue**. This custom build resolves several critical stability bugs and improves the visual experience of the game.

## Summary of Changes & Fixes

Below is a complete log of all modifications made to the base touchHLE engine to create this build:

### 1. Resolution, Window Upscaling, Unlocked Frame Rate
- Implemented a **1.5x rendering scale hack**.
- The emulator now automatically opens with a larger, upscaled window, removing the need for you to manually resize the game window every time you launch the emulator.
- The emulator now has an unlocked frame rate allowing for smoother gameplay with no impact on runtime.
  
### 2. Objective-C Null Pointer Safety Patch
- **The Bug**: The game would occasionally crash the emulator when trying to read or write properties on `nil` (non-existent) objects, particularly during combat attack animations.
- **The Fix**: Patched `objc_getProperty` and `objc_setProperty` inside the touchHLE runtime (`src/objc/properties.rs`) to safely detect and ignore `nil` objects without panicking, preventing random mid-battle crashes.

### 3. OpenAL Audio Resource Leak Fix
- **The Bug**: The game frequently creates audio queues for short sound effects (like battle attacks). The emulator was failing to delete the underlying OpenAL audio sources when these queues were disposed of. After a certain number of battles, the emulator would hit an `AL_OUT_OF_MEMORY` limit and crash abruptly.
- **The Fix**: Added missing `context.DeleteSources` cleanup logic to `AudioQueueDispose` inside `src/frameworks/audio_toolbox/audio_queue.rs`, completely eliminating the audio resource leak.

### 4. Speedhack
- Added a speedhack that user can change to allow fast-forwarding gameplay safely without compromising stability.
-  CONTROLS  - "]" SPEED UP  |   "[" SPEED DOWN

### 5. Crash Log Flushing Patch
- Upgraded the emulator's internal logging macro (`src/log.rs`) to instantly flush output to disk. This ensures that if the emulator ever encounters a hard crash in the future, the true error message is guaranteed to be saved in `touchHLE_log.txt` before the window closes.
### 6. Cave Rendering Fix
- **The Bug**: Deeper caves (like Pirate's Cave and Underground River) would fail to load their background sprites and render black screens when generating complex room layouts.
- **The Fix**: Modified the binary map configuration files to dynamically switch these bugged generic `Cave` archetypes to stable `Water` or `Dungeon` themes that have complete texture atlases.


### 7. Retina Graphics Natively Enabled
- **The Feature**: The game's native high-resolution (Retina) textures are now forced on by default within the emulator.
- **The Fix**: Overrode the `retina` flag in the emulator options to ensure crisp, high-quality sprite rendering without manual configuration.

### 8. Asset Dumping Memory Crash Fix
- **The Bug**: The emulator would aggressively dump the new high-resolution `.png` texture atlases to disk, rapidly exhausting memory and causing hard crashes on loading screens.
- **The Fix**: Added a filter to `ns_file_manager.rs` to intercept and quietly discard any `.png` asset extraction requests, stabilizing the memory footprint.

### 9. Tournament Timer Reduction (5 Seconds)
- **The Bug**: The tournament arena required a 24-hour real-time wait between battles.
- **The Fix**: Hex-patched the game's executable (`DragonIsland`) inside the `.ipa` to overwrite the hardcoded 24-hour floating-point constant (`86400.0`) with 5 seconds (`5.0`). The tournament now refreshes instantly.

## Setup & Usage

1. Extract the contents.
2. Run `touchHLE.exe` to launch the game. 
3. (Optional) Read `START HERE - How to set up.txt` for more detailed instructions on save files and setup.

---
*Built with ❤️ to keep Dragon Island Blue alive!*
