import appdirs
import os
import toml

from collections import UserDict


class Config(UserDict):

    def __init__(self, initialdata={}, path=None):
        super(Config, self).__init__(initialdata)

        if path is not None:
            self.path = path
        else:
            self.path = os.path.join(
                appdirs.user_config_dir('yokel', 'com.samwhited'),
                'yokelrc'
            )
        try:
            os.mkdir(os.path.dirname(self.path))
        except OSError:
            pass
        self.load()

    def load(self):
        if os.path.isfile(self.path):
            with open(self.path, 'r') as config_file:
                self.update(toml.loads(config_file.read()))

    def flush(self):
        with open(self.path, 'w+') as config_file:
            toml.dump(self.data, config_file)

    def exists(self):
        return os.path.exists(self.path)
