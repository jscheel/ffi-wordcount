require "benchmark"
require 'ffi'
require_relative 'word_counter'

if RUBY_PLATFORM.include?('darwin')
  EXT = 'dylib'
else
  EXT = 'so'
end

module Rust
  extend FFI::Library
  # TODO (jscheel): Change path for release.
  ffi_lib 'target/release/libwordcount.' + EXT

  class CountsArray < FFI::Struct
    layout :len,    :size_t, # dynamic array layout
           :data,   :pointer #

    def to_a
      self[:data].get_array_of_uint16(0, self[:len]).compact
    end
  end


  attach_function :word_counts, [:string], CountsArray.by_value
  # attach_function :word_counts_free, [:pointer], :void
end

puts "================"
puts "Starting Test..."

puts "Generating Data..."
text = 'Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium.'
body = ""
for i in 0..100 do
  body = body + text + "\n\n"
end

puts "Running Ruby..."
ruby_counts = []
time = Benchmark.measure do
  (1..1000).each { |i| ruby_counts.push(WordCounter.new(body).by_paragraph(limit: 100)) }
end
puts time

puts "Running Rust..."
rust_counts = []
time = Benchmark.measure do
  (1..1000).each do
    rust_counts.push Rust.word_counts(body).to_a
    # Rust.word_counts_free(ptr)
  end
end
puts time

puts "Result data the same? " + (rust_counts == ruby_counts ? 'yes' : 'no')

puts "================"
