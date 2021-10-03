from PyQt5.QtWidgets import QApplication, QLabel
from PyQt5.QtGui import QIcon
from PyQt5.QtCore import pyqtSlot

app = QApplication([])
label = QLabel('Hello World!')
label.show()
app.exec_()
