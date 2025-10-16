@echo off
REM Build the project in release mode
cargo build --release

REM Create a directory for the itch.io release
set OUTPUT_DIR=itchio_release
if exist %OUTPUT_DIR% rmdir /s /q %OUTPUT_DIR%
mkdir %OUTPUT_DIR%

REM Copy the executable and assets to the release directory
copy target\release\bevy_platformer.exe %OUTPUT_DIR%\
xcopy "assets" %OUTPUT_DIR%\assets /E /I

REM Add a README file
copy README.md %OUTPUT_DIR%\README.txt

REM Compress the release directory into a zip file
powershell Compress-Archive -Path %OUTPUT_DIR%\* -DestinationPath bevy_platformer_itchio.zip

@echo Project is ready for publishing on itch.io. Upload 'bevy_platformer_itchio.zip' to your itch.io page.