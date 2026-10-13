/****************************************************************************
** Meta object code from reading C++ file 'keyboard_interceptor.h'
**
** Created by: The Qt Meta Object Compiler version 69 (Qt 6.12.0)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include "../../../include/keyboard_interceptor.h"
#include <QtCore/qmetatype.h>

#include <QtCore/qtmochelpers.h>

#include <memory>


#include <QtCore/qxptype_traits.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'keyboard_interceptor.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 69
#error "This file was generated using the moc from 6.12.0. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

#ifndef Q_CONSTINIT
#define Q_CONSTINIT
#endif

QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
QT_WARNING_DISABLE_GCC("-Wuseless-cast")
namespace {
struct qt_meta_tag_ZN19KeyboardInterceptorE_t {};
} // unnamed namespace

template <> constexpr inline auto KeyboardInterceptor::qt_create_metaobjectdata<qt_meta_tag_ZN19KeyboardInterceptorE_t>()
{
    namespace QMC = QtMocConstants;
    QtMocHelpers::StringRefStorage qt_stringData {
        "KeyboardInterceptor",
        "interceptorStarted",
        "",
        "interceptorStopped",
        "permissionError",
        "devicePath",
        "keyEventProcessed",
        "evdevCode",
        "pressed"
    };

    QtMocHelpers::UintData qt_methods {
        // Signal 'interceptorStarted'
        QtMocHelpers::SignalData<void()>(1, 2, QMC::AccessPublic, QMetaType::Void),
        // Signal 'interceptorStopped'
        QtMocHelpers::SignalData<void()>(3, 2, QMC::AccessPublic, QMetaType::Void),
        // Signal 'permissionError'
        QtMocHelpers::SignalData<void(const QString &)>(4, 2, QMC::AccessPublic, QMetaType::Void, {{
            { QMetaType::QString, 5 },
        }}),
        // Signal 'keyEventProcessed'
        QtMocHelpers::SignalData<void(int, bool)>(6, 2, QMC::AccessPublic, QMetaType::Void, {{
            { QMetaType::Int, 7 }, { QMetaType::Bool, 8 },
        }}),
    };
    QtMocHelpers::UintData qt_properties {
    };
    QtMocHelpers::UintData qt_enums {
    };
    return QtMocHelpers::metaObjectData<KeyboardInterceptor, qt_meta_tag_ZN19KeyboardInterceptorE_t>(QMC::MetaObjectFlag{}, qt_stringData,
            qt_methods, qt_properties, qt_enums);
}
Q_CONSTINIT const QMetaObject KeyboardInterceptor::staticMetaObject = { {
    QMetaObject::SuperData::link<QThread::staticMetaObject>(),
    qt_staticMetaObjectStaticContent<qt_meta_tag_ZN19KeyboardInterceptorE_t>.stringdata,
    qt_staticMetaObjectStaticContent<qt_meta_tag_ZN19KeyboardInterceptorE_t>.data,
    qt_static_metacall,
    nullptr,
    qt_staticMetaObjectRelocatingContent<qt_meta_tag_ZN19KeyboardInterceptorE_t>.metaTypes,
    nullptr
} };

void KeyboardInterceptor::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    auto *_t = static_cast<KeyboardInterceptor *>(_o);
    if (_c == QMetaObject::InvokeMetaMethod) {
        switch (_id) {
        case 0: _t->interceptorStarted(); break;
        case 1: _t->interceptorStopped(); break;
        case 2: _t->permissionError((*reinterpret_cast<std::add_pointer_t<QString>>(_a[1]))); break;
        case 3: _t->keyEventProcessed((*reinterpret_cast<std::add_pointer_t<int>>(_a[1])),(*reinterpret_cast<std::add_pointer_t<bool>>(_a[2]))); break;
        default: ;
        }
    }
    if (_c == QMetaObject::IndexOfMethod) {
        if (QtMocHelpers::indexOfMethod<void (KeyboardInterceptor::*)()>(_a, &KeyboardInterceptor::interceptorStarted, 0))
            return;
        if (QtMocHelpers::indexOfMethod<void (KeyboardInterceptor::*)()>(_a, &KeyboardInterceptor::interceptorStopped, 1))
            return;
        if (QtMocHelpers::indexOfMethod<void (KeyboardInterceptor::*)(const QString & )>(_a, &KeyboardInterceptor::permissionError, 2))
            return;
        if (QtMocHelpers::indexOfMethod<void (KeyboardInterceptor::*)(int , bool )>(_a, &KeyboardInterceptor::keyEventProcessed, 3))
            return;
    }
}

const QMetaObject *KeyboardInterceptor::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *KeyboardInterceptor::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_staticMetaObjectStaticContent<qt_meta_tag_ZN19KeyboardInterceptorE_t>.strings))
        return static_cast<void*>(this);
    return QThread::qt_metacast(_clname);
}

int KeyboardInterceptor::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QThread::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 4)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 4;
    }
    if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 4)
            *reinterpret_cast<QMetaType *>(_a[0]) = QMetaType();
        _id -= 4;
    }
    return _id;
}

// SIGNAL 0
void KeyboardInterceptor::interceptorStarted()
{
    QMetaObject::activate(this, &staticMetaObject, 0, nullptr);
}

// SIGNAL 1
void KeyboardInterceptor::interceptorStopped()
{
    QMetaObject::activate(this, &staticMetaObject, 1, nullptr);
}

// SIGNAL 2
void KeyboardInterceptor::permissionError(const QString & _t1)
{
    QMetaObject::activate<void>(this, &staticMetaObject, 2, nullptr, _t1);
}

// SIGNAL 3
void KeyboardInterceptor::keyEventProcessed(int _t1, bool _t2)
{
    QMetaObject::activate<void>(this, &staticMetaObject, 3, nullptr, _t1, _t2);
}
QT_WARNING_POP
