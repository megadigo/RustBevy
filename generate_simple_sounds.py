# Simple WAV file generator without external dependencies
# This creates basic sine wave sounds

import wave
import math
import struct

def generate_wav_file(filename, frequency, duration, sample_rate=44100):
    """Generate a simple WAV file with a sine wave"""
    frames = int(duration * sample_rate)
    
    # Create WAV file
    with wave.open(filename, 'w') as wav_file:
        wav_file.setnchannels(1)  # Mono
        wav_file.setsampwidth(2)  # 16-bit
        wav_file.setframerate(sample_rate)
        
        for i in range(frames):
            # Generate sine wave
            value = math.sin(2 * math.pi * frequency * i / sample_rate)
            # Apply fade out
            envelope = max(0, 1 - (i / frames))
            value = value * envelope * 0.3  # Volume control
            # Convert to 16-bit integer
            data = struct.pack('<h', int(value * 32767))
            wav_file.writeframes(data)

# Generate sound files
generate_wav_file('assets/jump.wav', 440, 0.2)      # Jump: 440Hz, 0.2s
generate_wav_file('assets/collect.wav', 880, 0.3)   # Collect: 880Hz, 0.3s  
generate_wav_file('assets/death.wav', 220, 0.5)     # Death: 220Hz, 0.5s

print("Generated sound files:")
print("- assets/jump.wav")
print("- assets/collect.wav")
print("- assets/death.wav")