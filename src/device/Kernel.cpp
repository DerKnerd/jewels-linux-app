//
// Created by imanuel on 20.05.24.
//

#include "device.h"

namespace jewels::device {

auto Kernel::toJson() const -> QJsonObject {
  QJsonObject json;
  json["architecture"] = architecture;
  json["version"] = version;

  return json;
}
} // namespace jewels::collector
