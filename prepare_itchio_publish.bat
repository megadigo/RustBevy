@echo off
REM Build the project in release mode for optimal performance
cargo build --release

REM Copy the optimized executable to the existing itchio_release directory
copy target\release\bevy_platformer.exe itchio_release\bevy_platformer.exe

REM Ensure assets are up to date
xcopy "assets" "itchio_release\assets" /E /I /Y

REM Create version-tagged zip file for publishing
powershell Compress-Archive -Path itchio_release\* -DestinationPath bevy_platformer_v2.1_release.zip -Force

@echo Project is ready for publishing on itch.io. 
@echo Upload 'bevy_platformer_v2.1_release.zip' to your itch.io page.
@echo.
@echo Contents included:
@echo - bevy_platformer.exe (optimized release build)
@echo - assets\ folder with audio files
@echo - README.txt with game documentation