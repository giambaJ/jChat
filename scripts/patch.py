""" Save all changes to the jChat submodule in a patch file """
from os import chdir, system

chdir("chat")

system("git diff > ../patches/chat.patch")
