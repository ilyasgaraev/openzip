require "fiddle"
require "fiddle/import"

module Openzip
  extend Fiddle::Importer

  def self.lib_ext
    raise "Sorry, Windows is not supported" if RUBY_PLATFORM =~ /win32/

    RUBY_PLATFORM =~ /darwin/ ? "dylib" : "so"
  end

  dlload "#{File.dirname(__FILE__)}/../source/target/release/libopenzip.#{lib_ext}"

  def self.extract(zippath, outdirpath)
    extract_rust(zippath, outdirpath) != 0
  end

  extern "char extract_rust(char*, char*)"
  private_class_method :extract_rust
end
