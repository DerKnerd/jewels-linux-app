//
// Created by imanuel on 20.05.24.
//

#include "device.h"
#include <QJsonObject>
#include <iostream>

namespace jewels::device {
auto readFile(const QString &path) -> QString {
  QFile file(path);
  if (file.open(QIODevice::ReadOnly) && file.isOpen() && file.isReadable() &&
      file.exists()) {
    file.reset();
    QTextStream stream{&file};

    return stream.readAll().trimmed();
  } else {
    qWarning() << path.toStdString() << ": " << file.errorString().toStdString();
  }

  return "Unbekannt";
}

auto readDmi(const QString &dmi) -> QString {
  auto path = u"/sys/class/dmi/id/"_s;

  return readFile(path.append(dmi));
}
} // namespace jewels::device
