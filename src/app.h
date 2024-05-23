#pragma once

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
  Q_INVOKABLE void
  restoreWindowGeometry(QQuickWindow *window,
                        const QString &group = QStringLiteral("main")) const;
  // Save current window geometry
  Q_INVOKABLE void
  saveWindowGeometry(QQuickWindow *window,
                     const QString &group = QStringLiteral("main")) const;

  Q_INVOKABLE void logout();

  Q_INVOKABLE void login(const QString &path);

  Q_INVOKABLE void login(const QString &url, const QString &token);

  Q_PROPERTY(QString host READ host NOTIFY hostChanged)

  Q_PROPERTY(bool loggedIn READ loggedIn NOTIFY loggedInChanged)

  QString host();

  bool loggedIn();

signals:
  void hostChanged();

  void loggedInChanged();
};
} // namespace jewels