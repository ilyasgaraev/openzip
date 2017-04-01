# Openzip

[![Build Status](https://img.shields.io/travis/ilyasgaraev/openzip/master.svg?style=flat-square)](https://travis-ci.org/ilyasgaraev/openzip)
[![Code Climate](https://img.shields.io/codeclimate/github/ilyasgaraev/openzip.svg?style=flat-square)](https://codeclimate.com/github/ilyasgaraev/openzip)
[![Test Coverage](https://img.shields.io/codeclimate/coverage/github/ilyasgaraev/openzip.svg?style=flat-square)](https://codeclimate.com/github/ilyasgaraev/openzip/coverage)
[![Gem](https://img.shields.io/gem/v/openzip.svg?style=flat-square)](https://github.com/ilyasgaraev/openzip)


**Openzip** is a Ruby library (written in Rust) for fast extract Zip files.

## Usage

Before you begin, you need to install Rust (with Cargo) on your system (see [Requirements](#requirements)).

```ruby
Openzip.extract("path/to/file.zip", "extract/path")
# returns true if the method successfully executed; otherwise, false
```

The **DEBUG** environment variable can be used to enable debug logs:

```ruby
# DEBUG=true
Openzip.extract("wrong/path/file.zip", "extract/path")
# Error: No such file or directory (os error 2)
# => false
```

## Requirements

* Rust and Cargo ([https://www.rust-lang.org/en-US/install.html](https://www.rust-lang.org/en-US/install.html))
* Ruby 2.0 or greater

## Benchmarking

* Memory usage: [benchmarking/memory.md](benchmarking/memory.md)
* Iterations per second: [benchmarking/iterations.md](benchmarking/iterations.md)

## Installation

Add this line to your application's Gemfile:

```ruby
gem "openzip"
```

And then execute:

    $ bundle

Or install it yourself as:

    $ gem install openzip

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/ilyasgaraev/openzip. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [Contributor Covenant](http://contributor-covenant.org) code of conduct.


## License

The gem is available as open source under the terms of the [MIT License](http://opensource.org/licenses/MIT).
