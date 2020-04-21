from ctypes import cdll

lib = cdll.LoadLibrary("/Users/ucucmacmini/Workspace/code/rust/rustInsideOtherLanguages/embed/target/release/libembed.dylib")

lib.process()

print("done!")
