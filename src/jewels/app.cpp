#include "app.h"
#include "jewelsconfig.h"
#include <KSharedConfig>
#include <KWindowConfig>
#include <QJsonDocument>
#include <QJsonObject>
#include <QQuickWindow>

namespace jewels {
using namespace Qt::Literals::StringLiterals;

[[maybe_unused]] auto App::restoreWindowGeometry(QQuickWindow *window,
                                const QString &group) -> void {
  KConfig dataResource(u"data"_s, KConfig::SimpleConfig,
                       QStandardPaths::AppDataLocation);
  KConfigGroup windowGroup(&dataResource, u"Window-"_s + group);
  KWindowConfig::restoreWindowSize(window, windowGroup);
  KWindowConfig::restoreWindowPosition(window, windowGroup);
}

[[maybe_unused]] auto App::saveWindowGeometry(QQuickWindow *window,
                             const QString &group) -> void {
  KConfig dataResource(u"data"_s, KConfig::SimpleConfig,
                       QStandardPaths::AppDataLocation);
  KConfigGroup windowGroup(&dataResource, u"Window-"_s + group);
  KWindowConfig::saveWindowPosition(window, windowGroup);
  KWindowConfig::saveWindowSize(window, windowGroup);
  dataResource.sync();
}

} // namespace jewels

#include "moc_app.cpp"
