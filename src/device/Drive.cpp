//
// Created by imanuel on 20.05.24.
//

#include "device.h"

namespace jewels::device {
auto Drive::toJson() const -> QJsonObject {
  QJsonObject json;
  json["model"] = this->model;
  json["manufacturer"] = this->manufacturer;
  json["size"] = this->size;
  json["name"] = this->name;

  return json;
}
} // namespace jewels::device