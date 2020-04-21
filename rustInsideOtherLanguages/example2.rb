require 'ffi'

module Hello
  extend FFI::Library
  ffi_lib '/Users/ucucmacmini/Workspace/code/rust/rustInsideOtherLanguages/embed/target/release/libembed.dylib'
  attach_function :process, [], :void
end

Hello.process

puts 'done!'
