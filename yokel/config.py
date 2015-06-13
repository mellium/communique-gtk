import appdirs
import os

from collections import UserDict

import yaml


class Config(UserDict):

    def __init__(self, initialdata={}, path=None):
        super(Config, self).__init__(initialdata)

        if path is not None:
            self.path = path
        else:
            self.path = os.path.join(
                appdirs.user_config_dir('yokel', 'com.samwhited'),
                'config.yml'
            )
        try:
            os.mkdir(os.path.dirname(self.path))
        except OSError:
            pass
        self.load()

    def load(self):
        if os.path.isfile(self.path):
            with open(self.path, 'r') as config_file:
                self.update(yaml.load(config_file))
                # try:
                # except:
                #     pass

    def flush(self):
        with open(self.path, 'w+') as config_file:
            yaml.dump(self.data, config_file, default_flow_style=False)

    def exists(self):
        return os.path.exists(self.path)
