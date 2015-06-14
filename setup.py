import yokel

from setuptools import setup

setup(
    name=yokel.appname,
    version=yokel.version,
    description='A beautiful XMPP client in GTK3',
    author=yokel.appauthor,
    author_email='sam@samwhited.com',
    url='https://bitbucket.org/SamWhited/yokel',
    packages=['yokel'],
    require=[
        'appdirs',
        'sleekxmpp',
        'toml'
    ],
    tests_require=[
        'mock',
        'pytest',
        'tox',
        'virtualenv',
        'wheel',
        'pre-commit'
    ]
)
