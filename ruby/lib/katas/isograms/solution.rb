# frozen_string_literal: true

# https://www.codewars.com/kata/54ba84be607a92aa900000f1/ruby
module Isograms
  extend T::Sig

  sig { params(string: String).returns(T::Boolean) }
  def is_isogram(string)
    string.chars.map(&:downcase).sort.reduce('') do |last, char| 
      return false if last == char
      char
    end
    return true
  end
end
