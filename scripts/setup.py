from os import chdir, system

chdir("chat")

system("git apply ../patches/jChat.patch")
