# -*- coding: utf-8 -*-
import re
import sys

# import notebook
from notebook.notebookapp import main

if __name__ == "__main__":
    # notebook.notebookapp.trust_all()
    sys.argv[0] = re.sub(r"(-script\.pyw?|\.exe)?$", "", sys.argv[0])
    sys.exit(main())
