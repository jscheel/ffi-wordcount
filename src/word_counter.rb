class WordCounter

  BLOCK_ELEMENTS = %w(div p)

  def initialize(body)
    self.body = body
  end

  def by_paragraph(limit: nil)
    counts = []
    state = :space
    current_count = 0
    tag_buffer = nil

    end_of_paragraph = Proc.new do
      counts.push(current_count) if current_count > 0
      current_count = 0
      break if limit && counts.count > limit
    end

    body.each_char do |c|

      case c
      when "\n"
        case state
        # count last word of paragraph
        when :word then current_count += 1
        # two line breaks in a row ends paragraph
        when :line_break then end_of_paragraph.call
        end

        state = :line_break
      when " "
        # count word before space
        current_count += 1 if state == :word
        state = :space
      when "<"
        # count word before tag
        current_count += 1 if state == :word
        tag_buffer = StringIO.new
        state = :open_tag
      when ">"
        if state == :open_tag
          tag_buffer.rewind
          tag = tag_buffer.read

          is_br = tag == 'br'
          is_block_element = BLOCK_ELEMENTS.include?(tag)

          # go to next character if not line breaking tag
          unless is_br || is_block_element
            state = :html_close_tag
            next
          end

          if is_br && state == :line_break || is_block_element
            end_of_paragraph.call
          end

          state = :line_break
        else
          state = :word
        end
      else
        if state == :open_tag
          tag_buffer << c unless c == '/'
        else
          state = :word
        end
      end
    end

    # count last word in text if not wrapped in HTML
    current_count += 1 if state == :word
    counts.push(current_count) if current_count > 0

    counts
  end

  private

  attr_accessor :body

end
