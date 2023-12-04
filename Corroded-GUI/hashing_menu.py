import os
import sys
from PySide2.QtWidgets import *
from PySide2.QtCore import Qt, QSize

from PySide2.QtWidgets import QApplication, QMainWindow, QPushButton, QVBoxLayout, QLabel, QWidget, QDialog, QTextEdit
from PySide2.QtGui import QFont
from PySide2.QtCore import Qt, QSize

from ui_element_factory import UI_Element_Factory
from results_window import Results_Window
import interact

class Hashing_Menu:
    def __init__(self, widget_width=None):
        self.uief = UI_Element_Factory(widget_width)
        
        self.field1_label = "Input File"
        self.field1_tooltip = "Select the file you want to perform a sha-256 hash on"

        self.dialog = None

    def create_menu_dialog(self):
        self.dialog = QDialog()
        self.dialog.setWindowTitle("Hashing Menu")
        
        self.layout = QVBoxLayout()
        
        self.field1_hbox = self.uief.make_file_explorer_element(self.field1_label, self.field1_tooltip)
        self.layout.addLayout(self.field1_hbox)

        self.run_btn = self.uief.make_push_button_element(label="Hash", callback=self.run_hashing)
        self.layout.addWidget(self.run_btn)

        self.dialog.setLayout(self.layout)
        return self.dialog

    def show(self):
        if not self.dialog:
            self.create_menu_dialog()
        
        self.dialog.setModal(False)
        self.dialog.show()

    def run_hashing(self):
        file = self.uief.getValue(self.field1_label)
        
        hash_result = interact.hash(file)

        self.results_window = Results_Window("Hashing results")
        self.results_window.set_results_text(hash_result)
        self.results_window.show()



