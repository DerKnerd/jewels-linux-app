//
// Created by imanuel on 20.05.24.
//

#include "device.h"

namespace jewels::device {

auto Mainboard::toJson() const -> QJsonObject {
  QJsonObject json;
  json["manufacturer"] = manufacturer;
  json["version"] = version;
  json["model"] = model;

  return json;
}
} // namespace jewels::collector
