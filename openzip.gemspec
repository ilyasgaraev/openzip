# coding: utf-8

lib = File.expand_path("../lib", __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require "openzip/version"

Gem::Specification.new do |spec|
  spec.name          = "openzip"
  spec.version       = Openzip::VERSION
  spec.authors       = ["Ilyas Garaev"]
  spec.email         = ["vearagi@gmail.com"]

  spec.summary       = "Openzip is a Ruby library (written in Rust) for fast reading Zip files."
  spec.description   = "Openzip is a Ruby library (written in Rust) for fast reading Zip files."
  spec.homepage      = "https://github.com/ilyasgaraev/openzip"
  spec.license       = "MIT"

  spec.files         = `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^spec/}) }
  spec.test_files    = spec.files.grep(%r{^spec/})
  spec.require_paths = ["lib"]

  spec.extensions    = Dir["source/extconf.rb"]

  spec.required_ruby_version = "~> 2.0"

  spec.add_development_dependency "bundler", "~> 1.13"
  spec.add_development_dependency "bundler-audit", "~> 0.5"
  spec.add_development_dependency "pry", "~> 0.10"
  spec.add_development_dependency "rake", "~> 10.0"
  spec.add_development_dependency "rspec", "~> 3.5"
  spec.add_development_dependency "rubocop", "~> 0.47"
  spec.add_development_dependency "rubocop-rspec", "~> 1.0"
end
