# Openzip

[![Build Status](https://img.shields.io/travis/ilyasgaraev/openzip.svg?branch=master&style=flat-square)](https://travis-ci.org/ilyasgaraev/openzip)
[![Code Climate](https://img.shields.io/codeclimate/github/ilyasgaraev/openzip.svg?style=flat-square)](https://codeclimate.com/github/ilyasgaraev/openzip)
[![Test Coverage](https://img.shields.io/codeclimate/coverage/github/ilyasgaraev/openzip.svg?style=flat-square)](https://codeclimate.com/github/ilyasgaraev/openzip/coverage)
[![Gem](https://img.shields.io/gem/v/openzip.svg?style=flat-square)](https://github.com/ilyasgaraev/openzip)


**Openzip** is a Ruby library for fast extract Zip files.

## Usage

```ruby
path_to_zip = "path/to/zipfile.zip"
extract_to_path = "path/to/extract"

Openzip.extract(path_to_zip, extract_to_path)
```

## Installation

Add this line to your application's Gemfile:

```ruby
gem 'openzip'
```

And then execute:

    $ bundle

Or install it yourself as:

    $ gem install openzip

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/ilyasgaraev/openzip. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [Contributor Covenant](http://contributor-covenant.org) code of conduct.


## License

The gem is available as open source under the terms of the [MIT License](http://opensource.org/licenses/MIT).
