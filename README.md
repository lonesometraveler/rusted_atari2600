Rust Atari 2600 Emulator
========================

Building/Running:

    Install Rust:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh	
 
	For windows, see: https://www.rust-lang.org/tools/install

    Install SDL:
	linux (debian based): 
		apt-get install libsdl2-dev
	rasbian (64-bit): 
		apt-get install libsdl2-dev
	rasberry pi (ubuntu mate 64-bit): 
		# Release 22.04 LTS (Jammy Jellyfish) 64-bit
		# Need to upgrade so 'sdl2' will install.
		apt-get update
		apt-get upgrade
		apt-get install git curl libsdl2-dev

		# 'pipewire' appears to be a good sound driver on the raspberry pi
		# SDL_AUDIODRIVER=pipewire 
	OSX: 
		brew install sdl2

	Windows Build (from linux):
                sudo apt-get install gcc-mingw-w64
                rustup target add x86_64-pc-windows-gnu
                cargo build --target x86_64-pc-windows-gnu --release

                # For 'sdl'
                sudo apt-get install libsdl2-dev -y
                curl -s https://www.libsdl.org/release/SDL2-devel-2.0.22-mingw.tar.gz | tar xvz -C /tmp
                cp -r /tmp/SDL2-2.0.22/x86_64-w64-mingw32/lib/* ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib/

	Windows:
		Install a Visual Studio Compiler:
			https://visualstudio.microsoft.com/free-developer-offers/
		install 'cmake': 
			https://cmake.org/download/

		git clone https://github.com/libsdl-org/SDL
		git checkout release-2.28.5

		"C:\Program Files\CMake\bin\cmake.exe"cmake .. -DCMAKE_BUILD_TYPE=Release
		"C:\Program Files\CMake\bin\cmake.exe" --build . --config Release --parallel
                 copy Release\SDL2.* %USERPROFILE%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib

        Webassembly
                From: https://puddleofcode.com/story/definitive-guide-to-rust-sdl2-and-emscriptem
                Then: https://users.rust-lang.org/t/sdl2-emscripten-asmjs-and-invalid-renderer-panic/66567
                Taken from: https://github.com/therocode/rust_emscripten_main_loop


                sudo apt-get install emscripten
                rustup target add asmjs-unknown-emscripten

                # Your experience may vary, adding explicit handling of 'EM_CONFIG'

                EM_CONFIG=$HOME/.emscripten emcc --generate-config
                  Note: May have to manuall update/adjust available system versions in the '.emscripten' config file

                EM_CONFIG=~/.emscripten cargo build-emscripten
                  Note: It's just an alias for 'cargo build --release --config projects/emscripten/'

                # Start a web server and load in browser (point it to the location reported by the python server)
                python3 -m http.server

                # Drag a rom into the 'rom drop' location in the browser.

                # Note, the configuration file in 'projects/emscripten' are the same as running:
                export EMCC_CFLAGS="-s USE_SDL=2"
                cargo build --target asmjs-unknown-emscripten

Build and run:
    cargo run --release <rom_file>


    Usage: rusted_atari <cartridge_name> [-d] [-n] [-s <stop-clock>] [-f] [-l] [-r <replay-file>] [-c <cartridge-type>]

    Rusty Atari 2600 Emulator.
    
    Positional Arguments:
      cartridge_name    name of cartridge to run
    
    Options:
      -d, --debug       print PC State Debug Info
      -n, --no-delay    run the emulator with no delay (rather than real-time)
      -s, --stop-clock  number of clock cycles to stop the emulator (for
                        benchmarking)
      -f, --fullscreen  run the emulator in full screen mode.
      -p, --pal-palette use PAL palette (instead of NTSC)
      -l, --list-drivers
                        list SDL drivers
      -r, --replay-file replay file
      -c, --cartridge-type
                        cartridge type.  (Specifying an invalid option will display
                        available options).
      --help            display usage information


Somewhat working ROMs:
        https://forums.atariage.com/topic/206497-dk-vcs/

        'Version 1.0' is playable (although there are some querks).
        
        This uses cartridge type 'F4SC', so add the command line option '-c F4SC'

        Demo: 'Doctor by Trilobit', appears to work (after adding more (well documented) undocumented op-codes)

Rust dependencies:
        cargo add argh
        cargo add sdl2
        cargo add bitfield


PAL Colour palette from:
    https://www.qotile.net/minidig/docs/tia_color.html

    STELLA PROGRAMMER'S GUIDE
    by Steven Write (12/03/79)
        Great for most addressing/register information.

        The main gap is 'tone', which is hard to describe (and is acknowledge in the guide)
            '..Some are pure tones like a flute..'.  
            'Even though the TIA hardware manual lists the sounds, some experimentation will be necessary to find "your sound"'

    TIA Technical Manual
        TIA 1A, Television Interface Adaptor (Model 1A)

    TIA Hardware information:
        Atari 2600 TIA Hardware Notes
        Andrew Towers
        https://www.atarihq.com/danb/files/TIA_HW_Notes.txt

    Cartridge information:
        Cart Information
        Kevin Horton
        http://kevtris.org/files/sizes.txt

    TIA Schematics (mainly used to figure out sound polynomials, but also helped clarrify behaviours described in the 'hardware notes')
        https://atariage.com/2600/archives/schematics_tia/index.html

TODO:

Sound
    - Currently sound only somewhat works when running in realtime.  
      It 'can' run without any pops/crackle (added a sleep to tiasound audio, so time is linked to sound card fill buffer). 
      Ideally sound would be scaled, which would allow sound at faster/slower than real-time (as well as improve 'real-time')
      Note: Changing 'core::Constants::CLOCK_HZ' appears to lead to reasonable scaling overall (sound doesn't seem too bad and is simpler than separate sound scaling).

    - Only a subset of instructions have been implemented so far (intending to expand, but don't expect all cartridges to work).
    - Only the 'basic' cartridge format has been implemented (no bank switching/Ram).
    - No light gun support
    - No paddle support
    - No joystic 2
    - Initial horizontal blanking lines are display, they probably should be black (to better reflect the 'real world').

    - No 'audio selection' via command line (wav/sound is by changing comment in code).

    - Lots of other stuff, adding tests, re-working layout/dependency,

