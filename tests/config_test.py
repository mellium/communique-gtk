import mock
import pytest
import os

from yokel.config import Config


@pytest.yield_fixture
def config():
    with mock.patch('os.mkdir') as mkdir:
        with mock.patch.object(Config, 'load'):
            config = Config()
            assert mkdir.called_once_with(os.path.dirname(config.path))
            yield config


def test_default_file(config):
    """
    The default config file should be called 'yokelrc' (no matter where it's
    stored).
    """
    assert config.path.endswith('yokelrc')


def test_create_config(tmpdir):
    """
    OSError's should not be raised if the path to the file already exists,
    flushing should write the file, and loading should actually load in the
    data from the file.
    """
    config = Config(path="{}/yokelrc".format(tmpdir.realpath().strpath))
    assert config.exists() is False
    config['mikado'] = 'Short, sharp, shock'
    config.flush()
    assert config.exists() is True

    config_existing = Config(path=config.path)
    assert config_existing['mikado'] == config['mikado']
