# frozen_string_literal: true

module Inkoc
  module Type
    module Predicates
      def optional?
        false
      end

      def block?
        false
      end

      def regular_object?
        false
      end

      def generic_trait?
        false
      end

      def trait?
        false
      end

      def dynamic?
        false
      end

      def generated_trait?
        false
      end

      def self_type?
        false
      end
    end
  end
end
