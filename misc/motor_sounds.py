#!/usr/bin/env python3
"""
Motor Controller Sound Synthesizer
Recreates the beep sounds from the AM32 motor controller firmware
"""

import numpy as np
import sounddevice as sd
import time

# Constants from the C code
CPU_FREQUENCY_MHZ = 48  # Typical for ARM Cortex-M0
TIM1_AUTORELOAD = 2000
TIMER1_MAX_ARR = 2000


class MotorSounds:
    def __init__(self, sample_rate=44100, volume=0.3):
        """
        Initialize the sound synthesizer

        Args:
            sample_rate: Audio sample rate in Hz
            volume: Volume level (0.0 to 1.0)
        """
        self.sample_rate = sample_rate
        self.volume = volume
        self.beep_volume = 11 * 2  # Default volume from setVolume(11)

    def generate_tone(self, frequency, duration_ms, duty_cycle_ratio=0.5):
        """
        Generate a square wave tone

        Args:
            frequency: Frequency in Hz
            duration_ms: Duration in milliseconds
            duty_cycle_ratio: Duty cycle (0.0 to 1.0)
        """
        duration_s = duration_ms / 1000.0
        t = np.linspace(0, duration_s, int(self.sample_rate * duration_s), False)

        # Generate square wave
        wave = np.sin(2 * np.pi * frequency * t)
        square_wave = np.sign(wave)

        # Apply duty cycle by zeroing out portions of the wave
        phase = (frequency * t) % 1.0
        square_wave[phase > duty_cycle_ratio] = -1

        # Apply volume scaling
        return square_wave * self.volume

    def silence(self, duration_ms):
        """Generate silence for the specified duration"""
        duration_s = duration_ms / 1000.0
        return np.zeros(int(self.sample_rate * duration_s))

    def prescaler_to_frequency(self, prescaler):
        """
        Convert prescaler value to frequency (Hz)
        The PWM frequency is: CPU_FREQ / ((prescaler + 1) * TIM1_AUTORELOAD)
        """
        if prescaler == 0:
            return 0
        return (CPU_FREQUENCY_MHZ * 1000000) / ((prescaler + 1) * TIM1_AUTORELOAD)

    def play_audio(self, audio_data):
        """Play audio data through the default output device"""
        sd.play(audio_data, self.sample_rate)
        sd.wait()

    def play_startup_tune(self):
        """Recreate the playStartupTune() function"""
        print("Playing startup tune (3 ascending beeps)...")

        # First beep - prescaler 55
        freq1 = self.prescaler_to_frequency(55)
        beep1 = self.generate_tone(freq1, 200)

        # Second beep - prescaler 40
        freq2 = self.prescaler_to_frequency(40)
        beep2 = self.generate_tone(freq2, 200)

        # Third beep - prescaler 25
        freq3 = self.prescaler_to_frequency(25)
        beep3 = self.generate_tone(freq3, 200)

        # Combine all beeps
        audio = np.concatenate([beep1, beep2, beep3])
        self.play_audio(audio)

        print(f"  Beep 1: {freq1:.1f} Hz")
        print(f"  Beep 2: {freq2:.1f} Hz")
        print(f"  Beep 3: {freq3:.1f} Hz")

    def play_brushed_startup_tune(self):
        """Recreate the playBrushedStartupTune() function"""
        print("Playing brushed startup tune (4 ascending beeps)...")

        beeps = []
        prescalers = [40, 30, 25, 20]

        for prescaler in prescalers:
            freq = self.prescaler_to_frequency(prescaler)
            beeps.append(self.generate_tone(freq, 300))
            print(f"  Beep: {freq:.1f} Hz")

        audio = np.concatenate(beeps)
        self.play_audio(audio)

    def play_dusking_tune(self):
        """Recreate the playDuskingTune() function"""
        print("Playing dusking tune (complex melody)...")

        # Sequence of prescaler values and durations from the code
        sequence = [
            (60, 200),
            (55, 150),
            (50, 150),
            (45, 100),
            (50, 100),
            (55, 100),
            (25, 200),
            (55, 150),
        ]

        beeps = []
        for prescaler, duration in sequence:
            freq = self.prescaler_to_frequency(prescaler)
            beeps.append(self.generate_tone(freq, duration))

        audio = np.concatenate(beeps)
        self.play_audio(audio)

    def play_input_tune(self):
        """Recreate the playInputTune() function"""
        print("Playing input tune (3 descending beeps)...")

        sequence = [(80, 100), (70, 100), (40, 100)]

        beeps = []
        for prescaler, duration in sequence:
            freq = self.prescaler_to_frequency(prescaler)
            beeps.append(self.generate_tone(freq, duration))
            print(f"  Beep: {freq:.1f} Hz")

        audio = np.concatenate(beeps)
        self.play_audio(audio)

    def play_input_tune2(self):
        """Recreate the playInputTune2() function"""
        print("Playing input tune 2 (3 ascending beeps)...")

        sequence = [(60, 75), (80, 75), (90, 75)]

        beeps = []
        for prescaler, duration in sequence:
            freq = self.prescaler_to_frequency(prescaler)
            beeps.append(self.generate_tone(freq, duration))
            print(f"  Beep: {freq:.1f} Hz")

        audio = np.concatenate(beeps)
        self.play_audio(audio)

    def play_default_tone(self):
        """Recreate the playDefaultTone() function"""
        print("Playing default tone (2 descending beeps)...")

        sequence = [(50, 150), (30, 150)]

        beeps = []
        for prescaler, duration in sequence:
            freq = self.prescaler_to_frequency(prescaler)
            beeps.append(self.generate_tone(freq, duration))
            print(f"  Beep: {freq:.1f} Hz")

        audio = np.concatenate(beeps)
        self.play_audio(audio)

    def play_changed_tone(self):
        """Recreate the playChangedTone() function"""
        print("Playing changed tone (2 beeps: low-high)...")

        sequence = [(40, 150), (80, 150)]

        beeps = []
        for prescaler, duration in sequence:
            freq = self.prescaler_to_frequency(prescaler)
            beeps.append(self.generate_tone(freq, duration))
            print(f"  Beep: {freq:.1f} Hz")

        audio = np.concatenate(beeps)
        self.play_audio(audio)

    def play_beacon_tune3(self):
        """Recreate the playBeaconTune3() function - descending sweep"""
        print("Playing beacon tune 3 (descending sweep)...")

        beeps = []
        for i in range(119, 0, -2):
            prescaler = 10 + (i // 2)
            freq = self.prescaler_to_frequency(prescaler)
            beeps.append(self.generate_tone(freq, 10))

        audio = np.concatenate(beeps)
        self.play_audio(audio)
        print(
            f"  Swept from ~{self.prescaler_to_frequency(69):.1f} Hz to ~{self.prescaler_to_frequency(10):.1f} Hz"
        )


def main():
    """Main function to demonstrate all sounds"""
    print("=" * 60)
    print("Motor Controller Sound Synthesizer")
    print("=" * 60)
    print()

    # Check if sounddevice is available
    try:
        import sounddevice as sd
    except ImportError:
        print("ERROR: sounddevice library not found!")
        print("Please install it with: pip install sounddevice")
        return

    sounds = MotorSounds(volume=0.3)

    menu = """
Available sounds:
1. Startup Tune (3 ascending beeps)
2. Brushed Startup Tune (4 ascending beeps)
3. Dusking Tune (complex melody)
4. Input Tune (3 descending beeps)
5. Input Tune 2 (3 ascending beeps)
6. Default Tone (2 descending beeps)
7. Changed Tone (low-high beeps)
8. Beacon Tune 3 (descending sweep)
9. Play All Sounds
0. Exit
"""

    while True:
        print(menu)
        choice = input("Select a sound (0-9): ").strip()
        print()

        if choice == "0":
            print("Goodbye!")
            break
        elif choice == "1":
            sounds.play_startup_tune()
        elif choice == "2":
            sounds.play_brushed_startup_tune()
        elif choice == "3":
            sounds.play_dusking_tune()
        elif choice == "4":
            sounds.play_input_tune()
        elif choice == "5":
            sounds.play_input_tune2()
        elif choice == "6":
            sounds.play_default_tone()
        elif choice == "7":
            sounds.play_changed_tone()
        elif choice == "8":
            sounds.play_beacon_tune3()
        elif choice == "9":
            print("Playing all sounds in sequence...\n")
            for i in range(1, 9):
                print(f"\n--- Sound {i} ---")
                if i == 1:
                    sounds.play_startup_tune()
                elif i == 2:
                    sounds.play_brushed_startup_tune()
                elif i == 3:
                    sounds.play_dusking_tune()
                elif i == 4:
                    sounds.play_input_tune()
                elif i == 5:
                    sounds.play_input_tune2()
                elif i == 6:
                    sounds.play_default_tone()
                elif i == 7:
                    sounds.play_changed_tone()
                elif i == 8:
                    sounds.play_beacon_tune3()
                time.sleep(0.5)
        else:
            print("Invalid choice. Please try again.")

        print()


if __name__ == "__main__":
    main()
