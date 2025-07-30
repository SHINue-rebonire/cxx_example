#include "reverse.h"
#include <stdexcept>

rust::Vec<uint8_t> transform(rust::Slice<const uint8_t> img_data)
{
  if (img_data.empty())
  {
    throw std::runtime_error("データが空です");
  }

  rust::Vec<uint8_t> output;
  output.reserve(img_data.size());

  for (size_t i = 0; i < img_data.size(); ++i)
  {
    auto pixel = img_data[i];
    output.push_back(255 - pixel);
  }

  return output;
}