#pragma once

#include <KConfigGroup>
#include <QObject>
#include <QQmlEngine>
#include <Qt>

class QQuickWindow;

namespace jewels {

class App : public QObject {
  Q_OBJECT
  QML_ELEMENT
  QML_SINGLETON

public:
  // Restore current window geometry
  Q_INVOKABLE [[maybe_unused]] void
  restoreWindowGeometry(QQuickWindow *window,
                        const QString &group = QStringLiteral("main"));

  // Save current window geometry
  Q_INVOKABLE [[maybe_unused]] void
  saveWindowGeometry(QQuickWindow *window,
                     const QString &group = QStringLiteral("main"));
};
} // namespace jewels