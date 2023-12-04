import sys
from PySide2.QtWidgets import *
from PySide2.QtCore import Qt, QSize

from PySide2.QtWidgets import QApplication, QMainWindow, QPushButton, QVBoxLayout, QLabel, QWidget, QDialog, QTextEdit
from PySide2.QtGui import QFont
from PySide2.QtCore import Qt, QSize

from ui_element_factory import UI_Element_Factory
from encryption_menu import Encryption_Menu
from decryption_menu import Decryption_Menu
from hashing_menu import Hashing_Menu

class Main_Menu(QMainWindow):
    def __init__(self, widget_width=None):
        super().__init__()
        self.setWindowTitle("Corroded Cipher")
        #self.resize(300, 300)
        self.uief = UI_Element_Factory(widget_width)
        
        self.layout = QVBoxLayout()
        self.layout.addStretch(1)
        self.layout.addWidget(self.make_title())

        self.encryption_menu_btn = self.uief.make_push_button_element("Encryption Menu", None, self.run_encryption)
        self.layout.addWidget(self.encryption_menu_btn)
        
        self.decryption_menu_btn = self.uief.make_push_button_element("Decryption Menu", None, self.run_decryption)
        self.layout.addWidget(self.decryption_menu_btn)
       
        self.hashing_menu_btn = self.uief.make_push_button_element("Hashing Menu", None, self.run_hashing)
        self.layout.addWidget(self.hashing_menu_btn)

        self.quit_btn = self.uief.make_push_button_element("Quit", None, self.quit_application)
        self.layout.addWidget(self.quit_btn)

        self.central_widget = QWidget()
        self.central_widget.setLayout(self.layout)
        self.setCentralWidget(self.central_widget)

    def run_encryption(self):
        self.encryption_menu = Encryption_Menu()
        self.encryption_menu.show()

    def run_decryption(self):
        self.decryption_menu = Decryption_Menu()
        self.decryption_menu.show()

    def run_hashing(self):
        self.hashing_menu = Hashing_Menu()
        self.hashing_menu.show()

    def quit_application(self):
        sys.exit()
    
    def make_title(self) -> QWidget:
        title = QLabel("Corroded Cipher")
        title_font = QFont()
        title_font.setBold(True)
        title_font.setPointSize(16)
        title.setFont(title_font)
        title.setAlignment(Qt.AlignCenter)
        return title

if __name__ == "__main__":
    app = QApplication(sys.argv)
    main_menu = Main_Menu()
    main_menu.show()
    sys.exit(app.exec_())
