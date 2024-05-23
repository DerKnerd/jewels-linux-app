//
// Created by imanuel on 20.05.24.
//

#pragma once

#include <QException>
#include <QFuture>
#include <QObject>
#include <QPromise>
#include <QQmlEngine>
#include <QThread>
#include <Qt>

namespace jewels {
class PushDataException : public QException {
public:
  QString message;
};

class Jewels : public QObject {
  Q_OBJECT
  QML_ELEMENT
  QML_SINGLETON

public:
  auto sendData() -> QPromise<void> *;
};
} // namespace jewels
