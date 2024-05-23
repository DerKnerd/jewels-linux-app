//
// Created by imanuel on 20.05.24.
//

#include "device.h"

namespace jewels::device {
auto Bios::toJson() const -> QJsonObject {
  QJsonObject json;
  json["manufacturer"] = manufacturer;
  json["version"] = version;

  return json;
}
} // namespace jewels::collector
