unless system("cargo --version") || system("rustc --version")
  raise SystemCallError, "You have to install Rust with Cargo (https://www.rust-lang.org/en-US/install.html)"
end
