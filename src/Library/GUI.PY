import tkinter as tk
from tkinter import messagebox, filedialog, colorchooser
#                                _________  __       __   _________
# ========================= #   | ________| | |      | | |___   ___|
#                           #  / /          | |      | |     | |
#          GENTWO           #  | |          | |      | |     | |
#      L i b r a r y        #  | |      ___ | |      | |     | |
#                           #  | |_____|  _| \ \_____/ /  ___| |___
# ========================= #  \_________/    \_______/  |_________|
#                                                                   
def CREATEWINDOW(width, height, windowname):
    try:
        window = tk.Tk()
        window.geometry(f"{width}x{height}")
        window.title(windowname)
        return window
    except Exception:
        return 0


def MAINLOOP(window_instance):
    try:
        window_instance.mainloop()
        return 1
    except Exception:
        return 0


def SETTITLE(window_instance, title):
    try:
        window_instance.title(title)
        return 1
    except Exception:
        return 0


def SETSIZE(window_instance, width, height):
    try:
        window_instance.geometry(f"{width}x{height}")
        return 1
    except Exception:
        return 0


def CLOSEWINDOW(window_instance):
    try:
        window_instance.destroy()
        return 1
    except Exception:
        return 0


def SETRESIZABLE(window_instance, width, height):
    try:
        window_instance.resizable(width, height)
        return 1
    except Exception:
        return 0


# =========================
# WIDGETS
# =========================

def CREATELABEL(window_instance, text):
    try:
        return tk.Label(window_instance, text=text)
    except Exception:
        return 0


def CREATEBUTTON(window_instance, text, command=None):
    try:
        return tk.Button(
            window_instance,
            text=text,
            command=command
        )
    except Exception:
        return 0


def CREATEENTRY(window_instance):
    try:
        return tk.Entry(window_instance)
    except Exception:
        return 0


def CREATETEXTBOX(window_instance, width=40, height=10):
    try:
        return tk.Text(
            window_instance,
            width=width,
            height=height
        )
    except Exception:
        return 0


def CREATECHECKBOX(window_instance, text, variable=None):
    try:
        if variable is None:
            variable = tk.BooleanVar(master=window_instance)

        widget = tk.Checkbutton(
            window_instance,
            text=text,
            variable=variable
        )
        return widget
    except Exception:
        return 0


def CREATERADIOBUTTON(window_instance, text, variable, value):
    try:
        return tk.Radiobutton(
            window_instance,
            text=text,
            variable=variable,
            value=value
        )
    except Exception:
        return 0


def CREATESCALE(window_instance, minimum, maximum, horizontal=True):
    try:
        orient = tk.HORIZONTAL if horizontal else tk.VERTICAL
        return tk.Scale(
            window_instance,
            from_=minimum,
            to=maximum,
            orient=orient
        )
    except Exception:
        return 0


# =========================
# LAYOUT
# =========================

def PACK(widget, **options):
    try:
        widget.pack(**options)
        return 1
    except Exception:
        return 0


def GRID(widget, row, column, **options):
    try:
        widget.grid(row=row, column=column, **options)
        return 1
    except Exception:
        return 0


def PLACE(widget, x, y, **options):
    try:
        widget.place(x=x, y=y, **options)
        return 1
    except Exception:
        return 0


# =========================
# WIDGET CONTROL
# =========================

def SETTEXT(widget, text):
    try:
        if isinstance(widget, tk.Entry):
            widget.delete(0, tk.END)
            widget.insert(0, text)
        elif isinstance(widget, tk.Text):
            widget.delete("1.0", tk.END)
            widget.insert("1.0", text)
        else:
            widget.configure(text=text)
        return 1
    except Exception:
        return 0


def GETTEXT(widget):
    try:
        if isinstance(widget, tk.Entry):
            return widget.get()
        elif isinstance(widget, tk.Text):
            return widget.get("1.0", "end-1c")
        else:
            return widget.cget("text")
    except Exception:
        return 0


def SETCOLOR(widget, foreground=None, background=None):
    try:
        options = {}
        if foreground is not None:
            options["fg"] = foreground
        if background is not None:
            options["bg"] = background

        widget.configure(**options)
        return 1
    except Exception:
        return 0


def SETSTATE(widget, enabled=True):
    try:
        widget.configure(
            state=tk.NORMAL if enabled else tk.DISABLED
        )
        return 1
    except Exception:
        return 0


def SETCOMMAND(widget, command):
    try:
        widget.configure(command=command)
        return 1
    except Exception:
        return 0


# =========================
# DIALOGS
# =========================

def SHOWMESSAGE(title, message):
    try:
        messagebox.showinfo(title, message)
        return 1
    except Exception:
        return 0


def SHOWWARNING(title, message):
    try:
        messagebox.showwarning(title, message)
        return 1
    except Exception:
        return 0


def SHOWERROR(title, message):
    try:
        messagebox.showerror(title, message)
        return 1
    except Exception:
        return 0


def ASKYESNO(title, message):
    try:
        return int(messagebox.askyesno(title, message))
    except Exception:
        return 0


def OPENFILE():
    try:
        return filedialog.askopenfilename()
    except Exception:
        return 0


def SAVEFILE():
    try:
        return filedialog.asksaveasfilename()
    except Exception:
        return 0


def CHOOSECOLOR():
    try:
        result = colorchooser.askcolor()
        return result[1] if result[1] is not None else 0
    except Exception:
        return 0
