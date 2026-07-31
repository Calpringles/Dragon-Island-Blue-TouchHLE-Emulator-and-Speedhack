HOW TO PLAY DRAGON ISLAND BLUE ON YOUR MAC
==========================================

This lets you play the old iPhone game Dragon Island Blue on a Mac.

It takes about five minutes. You do not need to be technical — just follow
the steps in order.

You need two things:
  * this folder (you already have it)
  * the game file, which ends in ".ipa"

Any Mac from about 2012 onwards will work.


- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
STEP 1 — Put the app somewhere permanent
- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

Drag "touchHLE" out of this folder and into your Applications folder.

(If you skip this it still works, but the app may vanish next time you tidy
your Downloads folder.)


- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
STEP 2 — Open it the first time (the fiddly bit — read this one carefully)
- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

The first time you open it, your Mac WILL refuse and show a warning. This is
normal and expected. It happens to every app that isn't sold through the App
Store. Nothing is wrong.

Here is how to get past it.

  1. Double-click touchHLE. A message appears saying it cannot be opened,
     or that Apple cannot check it for malicious software. Click "Done"
     or "OK".

  2. Open System Settings (the grey gear icon in your Dock, or the Apple
     menu in the top-left corner -> System Settings).

  3. In the left-hand list, click "Privacy & Security".

  4. Scroll down. Near the bottom you will see a line that says:

         "touchHLE" was blocked to protect your Mac.

     Next to it is a button: "Open Anyway". Click it.

  5. Enter your Mac password, or use Touch ID, if asked.

  6. A final warning appears. Click "Open Anyway" again.

The app now opens. You will never have to do any of this again.

  If you have an older Mac and step 4 shows nothing, try this instead:
  right-click (or hold Control and click) on touchHLE, choose "Open" from
  the menu, then click "Open" in the box that appears.


- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
STEP 3 — Give it the game
- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

When it opens, you will see a mostly empty screen saying "No games found".
That is correct — you haven't added the game yet.

  1. Click the "File manager" button at the bottom of the window.

  2. A Finder window opens, and touchHLE closes itself. This is deliberate.

  3. In that Finder window, double-click the folder called "touchHLE_apps".

  4. Drag your game file (the one ending in ".ipa") into that folder.

  5. Open touchHLE again (from Applications). The game is now shown as an
     icon. Click it, and it starts.

That's it. From now on, playing is just: open touchHLE, click the game.


- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
PLAYING
- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

The mouse is your finger. Click where you would tap. Click and drag where
you would swipe.

To make the window bigger, drag any edge or corner of it. The game was made
for a phone screen, so it will look softer the bigger you make it. That is
the game's real resolution, not a setting you can turn up.

To stop playing, close the window.


- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
IF SOMETHING GOES WRONG
- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

"It says the app is damaged and should be moved to the Bin."
    This happens if the download got mangled. Open the Terminal app
    (press Command-Space, type Terminal, press Enter), paste in the line
    below exactly, and press Enter. Then try opening the app again.

        xattr -dr com.apple.quarantine /Applications/touchHLE.app

"The game isn't showing up after I copied it in."
    Make sure the file ends in ".ipa", and that it is directly inside the
    "touchHLE_apps" folder rather than inside another folder within it.
    Then close touchHLE completely and open it again.

"Where are my saves?"
    In the same folder the "File manager" button opens, inside a folder
    called "touchHLE_sandbox". Copy that folder somewhere safe if you want
    to back up your progress.


- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

This is a modified version of touchHLE (https://touchhle.org), a free
open-source iPhone emulator, with extra fixes for this particular game. It
is free software under the Mozilla Public License 2.0. The game itself is
not included and is not part of this package.
