""" Setup environment by initializing submodules and applying patches """
from os import chdir, system

system("git submodule update --init --recursive")

chdir("chat")

system("git apply ../patches/jChat.patch")
