import os
import sys
from PySide2.QtWidgets import *
from PySide2.QtCore import Qt, QSize

from PySide2.QtWidgets import QApplication, QMainWindow, QPushButton, QVBoxLayout, QLabel, QWidget, QDialog, QTextEdit
from PySide2.QtGui import QFont
from PySide2.QtCore import Qt, QSize

from ui_element_factory import UI_Element_Factory
import interact

class Decryption_Menu:
    def __init__(self, widget_width=None):
        self.uief = UI_Element_Factory(widget_width)
        
        self.field1_label = "Algorithm"
        self.field1_tooltip = "Select what encryption algorithm was used"
        
        self.field2_label = "Input File"
        self.field2_tooltip = "Select the file you want to decrypt"

        self.field3_label = "Password"
        self.field3_tooltip = "Enter the password you want to decrypt with"

        self.field4_label = "Output destination"
        self.field4_tooltip = "Select the output destination where you want to save the decrypted file"

        self.dialog = None

    def create_menu_dialog(self):
        self.dialog = QDialog()
        self.dialog.setWindowTitle("Decryption Menu")
        
        self.layout = QVBoxLayout()
        
        options = ["AES128", "AES192", "AES256"]
        self.field1_hbox = self.uief.make_dropdown_entry_element(self.field1_label, options, self.field1_tooltip)
        self.layout.addLayout(self.field1_hbox)

        self.field2_hbox = self.uief.make_file_explorer_element(self.field2_label, self.field2_tooltip)
        self.layout.addLayout(self.field2_hbox)

        self.field3_hbox = self.uief.make_password_entry_element(self.field3_label, self.field3_tooltip)
        self.layout.addLayout(self.field3_hbox)

        self.field4_hbox = self.uief.make_directory_explorer_element(self.field4_label, self.field4_tooltip)
        self.layout.addLayout(self.field4_hbox)

        self.run_btn = self.uief.make_push_button_element(label="Decrypt", callback=self.run_decryption)
        self.layout.addWidget(self.run_btn)

        self.dialog.setLayout(self.layout)
        return self.dialog

    def show(self):
        if not self.dialog:
            self.create_menu_dialog()
        
        self.dialog.setModal(False)
        self.dialog.show()
    
    def run_decryption(self):
        suffix = ".decrypted"

        algo     = self.uief.getValue(self.field1_label)
        file     = self.uief.getValue(self.field2_label)
        password = self.uief.getValue(self.field3_label)
        output   = self.uief.getValue(self.field4_label)

        filename = os.path.basename(file)
        output = os.path.join(output, filename + suffix)

        print(f"{algo}\n{file}\n{password}\n{output}", file=sys.stderr)
        interact.decrypt(algo, file, password, output)
