/****************************************************************************
** Meta object code from reading C++ file 'controller_diagram.h'
**
** Created by: The Qt Meta Object Compiler version 69 (Qt 6.12.0)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include "../../../include/controller_diagram.h"
#include <QtCore/qmetatype.h>

#include <QtCore/qtmochelpers.h>

#include <memory>


#include <QtCore/qxptype_traits.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'controller_diagram.h' doesn't include <QObject>."
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
struct qt_meta_tag_ZN17ControllerDiagramE_t {};
} // unnamed namespace

template <> constexpr inline auto ControllerDiagram::qt_create_metaobjectdata<qt_meta_tag_ZN17ControllerDiagramE_t>()
{
    namespace QMC = QtMocConstants;
    QtMocHelpers::StringRefStorage qt_stringData {
        "ControllerDiagram",
        "bindingChanged",
        "",
        "VirtualGamepadButton",
        "button",
        "qtKey",
        "nativeScanCode",
        "keyName",
        "bindingCleared",
        "listeningStateChanged",
        "isListening"
    };

    QtMocHelpers::UintData qt_methods {
        // Signal 'bindingChanged'
        QtMocHelpers::SignalData<void(VirtualGamepadButton, int, quint32, const QString &)>(1, 2, QMC::AccessPublic, QMetaType::Void, {{
            { 0x80000000 | 3, 4 }, { QMetaType::Int, 5 }, { QMetaType::UInt, 6 }, { QMetaType::QString, 7 },
        }}),
        // Signal 'bindingCleared'
        QtMocHelpers::SignalData<void(VirtualGamepadButton)>(8, 2, QMC::AccessPublic, QMetaType::Void, {{
            { 0x80000000 | 3, 4 },
        }}),
        // Signal 'listeningStateChanged'
        QtMocHelpers::SignalData<void(bool)>(9, 2, QMC::AccessPublic, QMetaType::Void, {{
            { QMetaType::Bool, 10 },
        }}),
    };
    QtMocHelpers::UintData qt_properties {
    };
    QtMocHelpers::UintData qt_enums {
    };
    return QtMocHelpers::metaObjectData<ControllerDiagram, qt_meta_tag_ZN17ControllerDiagramE_t>(QMC::MetaObjectFlag{}, qt_stringData,
            qt_methods, qt_properties, qt_enums);
}
Q_CONSTINIT const QMetaObject ControllerDiagram::staticMetaObject = { {
    QMetaObject::SuperData::link<QWidget::staticMetaObject>(),
    qt_staticMetaObjectStaticContent<qt_meta_tag_ZN17ControllerDiagramE_t>.stringdata,
    qt_staticMetaObjectStaticContent<qt_meta_tag_ZN17ControllerDiagramE_t>.data,
    qt_static_metacall,
    nullptr,
    qt_staticMetaObjectRelocatingContent<qt_meta_tag_ZN17ControllerDiagramE_t>.metaTypes,
    nullptr
} };

void ControllerDiagram::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    auto *_t = static_cast<ControllerDiagram *>(_o);
    if (_c == QMetaObject::InvokeMetaMethod) {
        switch (_id) {
        case 0: _t->bindingChanged((*reinterpret_cast<std::add_pointer_t<VirtualGamepadButton>>(_a[1])),(*reinterpret_cast<std::add_pointer_t<int>>(_a[2])),(*reinterpret_cast<std::add_pointer_t<quint32>>(_a[3])),(*reinterpret_cast<std::add_pointer_t<QString>>(_a[4]))); break;
        case 1: _t->bindingCleared((*reinterpret_cast<std::add_pointer_t<VirtualGamepadButton>>(_a[1]))); break;
        case 2: _t->listeningStateChanged((*reinterpret_cast<std::add_pointer_t<bool>>(_a[1]))); break;
        default: ;
        }
    }
    if (_c == QMetaObject::IndexOfMethod) {
        if (QtMocHelpers::indexOfMethod<void (ControllerDiagram::*)(VirtualGamepadButton , int , quint32 , const QString & )>(_a, &ControllerDiagram::bindingChanged, 0))
            return;
        if (QtMocHelpers::indexOfMethod<void (ControllerDiagram::*)(VirtualGamepadButton )>(_a, &ControllerDiagram::bindingCleared, 1))
            return;
        if (QtMocHelpers::indexOfMethod<void (ControllerDiagram::*)(bool )>(_a, &ControllerDiagram::listeningStateChanged, 2))
            return;
    }
}

const QMetaObject *ControllerDiagram::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *ControllerDiagram::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_staticMetaObjectStaticContent<qt_meta_tag_ZN17ControllerDiagramE_t>.strings))
        return static_cast<void*>(this);
    return QWidget::qt_metacast(_clname);
}

int ControllerDiagram::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QWidget::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 3)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 3;
    }
    if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 3)
            *reinterpret_cast<QMetaType *>(_a[0]) = QMetaType();
        _id -= 3;
    }
    return _id;
}

// SIGNAL 0
void ControllerDiagram::bindingChanged(VirtualGamepadButton _t1, int _t2, quint32 _t3, const QString & _t4)
{
    QMetaObject::activate<void>(this, &staticMetaObject, 0, nullptr, _t1, _t2, _t3, _t4);
}

// SIGNAL 1
void ControllerDiagram::bindingCleared(VirtualGamepadButton _t1)
{
    QMetaObject::activate<void>(this, &staticMetaObject, 1, nullptr, _t1);
}

// SIGNAL 2
void ControllerDiagram::listeningStateChanged(bool _t1)
{
    QMetaObject::activate<void>(this, &staticMetaObject, 2, nullptr, _t1);
}
QT_WARNING_POP
