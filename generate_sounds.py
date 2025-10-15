"""
Simple Python script to generate basic sound effects as OGG files.
Run this with: python generate_sounds.py
"""

import numpy as np
import scipy.io.wavfile as wavfile
import os

def generate_beep(frequency, duration, sample_rate=44100, fade_out=True):
    """Generate a simple sine wave beep"""
    t = np.linspace(0, duration, int(sample_rate * duration))
    wave = np.sin(2 * np.pi * frequency * t)
    
    if fade_out:
        # Apply fade out envelope
        envelope = np.linspace(1, 0, len(wave))
        wave = wave * envelope
    
    # Apply volume and convert to 16-bit
    wave = (wave * 0.3 * 32767).astype(np.int16)
    return wave

def main():
    # Create assets directory if it doesn't exist
    os.makedirs("assets", exist_ok=True)
    
    # Generate jump sound (440Hz, 0.2 seconds)
    jump_sound = generate_beep(440, 0.2)
    wavfile.write("assets/jump.wav", 44100, jump_sound)
    
    # Generate collect sound (880Hz, 0.3 seconds)
    collect_sound = generate_beep(880, 0.3)
    wavfile.write("assets/collect.wav", 44100, collect_sound)
    
    # Generate death sound (220Hz, 0.5 seconds)
    death_sound = generate_beep(220, 0.5)
    wavfile.write("assets/death.wav", 44100, death_sound)
    
    print("Generated sound files:")
    print("- assets/jump.wav")
    print("- assets/collect.wav") 
    print("- assets/death.wav")
    print("\nNote: Bevy supports WAV files directly, so no need to convert to OGG")

if __name__ == "__main__":
    main()