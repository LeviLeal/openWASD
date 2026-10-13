#include "mainwindow.h"
#include <QApplication>
#include <QStyleFactory>
#include <QIcon>

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);
    app.setApplicationName("WASD Anywhere");
    app.setOrganizationName("WASDAnywhere");

    // Use Breeze style if available (KDE standard), fallback to Fusion
    QStringList styles = QStyleFactory::keys();
    bool hasBreeze = false;
    for (const QString &s : styles) {
        if (s.compare("Breeze", Qt::CaseInsensitive) == 0) {
            QApplication::setStyle(QStyleFactory::create(s));
            hasBreeze = true;
            break;
        }
    }
    if (!hasBreeze) {
        QApplication::setStyle(QStyleFactory::create("Fusion"));
    }

    // Application Icon from embedded resources with fallback
    QIcon appIcon(":/icon.png");
    if (appIcon.isNull()) {
        appIcon = QIcon::fromTheme("input-gaming", QIcon::fromTheme("input-gamepad"));
    }
    app.setWindowIcon(appIcon);

    MainWindow window;
    window.showMaximized();

    return app.exec();
}
