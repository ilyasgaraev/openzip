require "fiddle"
require "fiddle/import"

module Openzip
  extend Fiddle::Importer

  def self.lib_ext
    raise "Sorry, windows is not supported yet" if RUBY_PLATFORM =~ /win32/

    RUBY_PLATFORM =~ /darwin/ ? "dylib" : "so"
  end

  dlload "#{File.dirname(__FILE__)}/../source/target/release/libopenzip.#{lib_ext}"
  extern "void extract(char*, char*)"
end
