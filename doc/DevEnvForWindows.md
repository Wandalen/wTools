If you got: `error: couldn't read PATH_TO_OUR_LIBRARY: The system cannot find the path specified. (os error 3)`:

1) Enable Dev mod

WINDOWS 10/11:
> 1) Open the "Settings" app by clicking on the "Start" button and then clicking on the gear icon.
> 2) Click on "Update & Security" from the options.
> 3) Select "For developers" from the left-hand menu.
> 4) Click the "Developer mode" radio button.
> 5) If prompted, confirm the action by clicking "Yes" to allow changes to your device.

2) `git config --global core.symlinks true`

3) `cargo clean` and remove `Cargo.lock`

4) Remove already loaded library. Beacause it has broken symlinks.

By default it is on path:
- if it was git link [ E.g. `wtools = { git = "<URL>", branch = "<BRANCH_NAME>" }` ]
> `C:\Users\<username>\.cargo\git\checkouts\wtools-*\`
- if it was simple dependency [ E.g. `wtools = "*"` ]
> `C:\Users\<username>\.cargo\registry\cache`<br>
> `C:\Users\<username>\.cargo\registry\src`

5) Build it the way you do.
