import importlib
import bevying

importlib.reload(bevying)
from bevying.bevying import Bevy


CustomParHelper: CustomParHelper = (
    next(d for d in me.docked if "ExtUtils" in d.tags)
    .mod("CustomParHelper")
    .CustomParHelper
)  # import
NoNode: NoNode = (
    next(d for d in me.docked if "ExtUtils" in d.tags).mod("NoNode").NoNode
)  # import
###


class BevyExt:
    def __init__(self, ownerComp):
        self.ownerComp = ownerComp
        CustomParHelper.Init(
            self, ownerComp, enable_properties=True, enable_callbacks=True
        )
        NoNode.Init(
            ownerComp,
            enable_chopexec=True,
            enable_datexec=True,
            enable_parexec=True,
            enable_keyboard_shortcuts=True,
        )
        self.Bevy = Bevy()
