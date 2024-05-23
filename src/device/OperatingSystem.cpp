//
// Created by imanuel on 20.05.24.
//

#include "device.h"
#include <QSettings>

namespace jewels::device {

auto OperatingSystem::toJson() const -> QJsonObject {
  QJsonObject json;
  json["name"] = name;
  json["version"] = version;

  return json;
}
} // namespace jewels::device