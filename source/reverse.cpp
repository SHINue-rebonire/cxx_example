#include "reverse.h"

rust::Vec<uint8_t> transform(rust::Slice<const uint8_t> img_data)
{
  rust::Vec<uint8_t> output;
  output.reserve(img_data.size());

  for (const auto &pixel : img_data)
  {
    output.push_back(255 - pixel);
  }

  return output;
}