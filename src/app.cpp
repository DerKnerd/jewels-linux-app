#include "app.h"
#include "jewelsconfig.h"
#include <KSharedConfig>
#include <KWindowConfig>
#include <QFile>
#include <QJsonDocument>
#include <QJsonObject>
#include <QQuickWindow>

namespace jewels {
using namespace Qt::Literals::StringLiterals;

auto App::restoreWindowGeometry(QQuickWindow *window,
                                const QString &group) const -> void {
  KConfig dataResource(QStringLiteral("data"), KConfig::SimpleConfig,
                       QStandardPaths::AppDataLocation);
  KConfigGroup windowGroup(&dataResource, QStringLiteral("Window-") + group);
  KWindowConfig::restoreWindowSize(window, windowGroup);
  KWindowConfig::restoreWindowPosition(window, windowGroup);
}

auto App::saveWindowGeometry(QQuickWindow *window,
                             const QString &group) const -> void {
  KConfig dataResource(QStringLiteral("data"), KConfig::SimpleConfig,
                       QStandardPaths::AppDataLocation);
  KConfigGroup windowGroup(&dataResource, QStringLiteral("Window-") + group);
  KWindowConfig::saveWindowPosition(window, windowGroup);
  KWindowConfig::saveWindowSize(window, windowGroup);
  dataResource.sync();
}

auto App::logout() -> void {
  JewelsConfig::setHost(QStringLiteral(""));
  JewelsConfig::setToken(QStringLiteral(""));
  this->loggedInChanged();
  this->hostChanged();
}

auto App::login(const QString &path) -> void {
  auto jsonDoc = QJsonDocument::fromJson(QFile(path).readAll()).object();
  JewelsConfig::setHost(jsonDoc.value(u"host"_s).toString());
  JewelsConfig::setToken(jsonDoc.value(u"token"_s).toString());

  auto config = JewelsConfig::self();
  config->save();

  this->loggedInChanged();
  this->hostChanged();
}

auto App::login(const QString &url, const QString &token) -> void {
  JewelsConfig::setHost(url);
  JewelsConfig::setToken(token);

  auto config = JewelsConfig::self();
  config->save();

  this->loggedInChanged();
  this->hostChanged();
}

auto App::host() -> QString {
  return JewelsConfig::host()
      .replace(u"https://"_s, u""_s)
      .replace(u"http://"_s, u""_s);
}

auto App::loggedIn() -> bool {
  this->loggedInChanged();

  return !this->host().isEmpty();
}
} // namespace jewels

#include "moc_app.cpp"
