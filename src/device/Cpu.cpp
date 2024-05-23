//
// Created by imanuel on 20.05.24.
//

#include "device.h"

namespace jewels::device {

auto Cpu::toJson() const -> QJsonObject {
  QJsonObject json;
  json["model"] = this->model;
  json["manufacturer"] = this->manufacturer;
  json["threads"] = this->threads;
  json["speed"] = this->speed;
  json["cores"] = this->cores;

  return json;
}
} // namespace jewels::device