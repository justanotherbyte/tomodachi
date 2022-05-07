import voice
from tomodachi import __version__


def test_version():
    assert __version__ == '0.1.0'

def test_recording():
    voice.record_audio("test.wav", 3)

def test_speak():
    voice.speak("PyTest voice test")