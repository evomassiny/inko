# frozen_string_literal: true

module Inkoc
  module TIR
    module Instruction
      class GetStringPrototype
        include Predicates
        include Inspect

        attr_reader :register, :location

        def initialize(register, location)
          @register = register
          @location = location
        end

        def visitor_method
          :on_get_string_prototype
        end
      end
    end
  end
end